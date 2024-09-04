<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.005.001 Post Create Member</name>
   <tag></tag>
   <elementGuidId>6c330b28-ca5f-4eee-8c52-c0b383742927</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;fullName\&quot;: \&quot;user3_mailnesia\&quot; ,\n    \&quot;nickName\&quot;: \&quot;mailnesia\&quot;,\n    \&quot;employeeType\&quot;: \&quot;Tetap\&quot;,\n    \&quot;finishedDate\&quot;: \&quot;2011-10-05T14:48:00.000Z\&quot;,\n    \&quot;division\&quot;: \&quot;RND\&quot;,\n    \&quot;position\&quot;: \&quot;Sales\&quot;,\n    \&quot;team\&quot;: null ,\n    \&quot;email\&quot;: \&quot;user3@mailnesia.com\&quot;,\n    \&quot;phoneNumber\&quot;: \&quot;+62 12345675\&quot;,\n    \&quot;groupMember\&quot;: \&quot;66ce902fdd806a71d2fa4896\&quot;,\n    \&quot;initial\&quot;: \&quot;RDG\&quot;,\n    \&quot;adminRole\&quot;: null,\n    \&quot;needLeaderReview\&quot;: true,\n    \&quot;needTestingAccount\&quot;: true,\n    \&quot;figmaPassword\&quot;: \&quot;figmaPassword\&quot;,\n    \&quot;githubPassword\&quot;: \&quot;figmaPassword4\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>b02f5679-bc31-4b46-8d42-b78bb34e8ce5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${list-account-member}${endpoint}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.team_member_member</defaultValue>
      <description></description>
      <id>febfa1bd-a0ab-4582-af2e-499fbf137de0</id>
      <masked>false</masked>
      <name>list-account-member</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>397a2051-f447-4ced-9a4e-c9ce5a5a74dc</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'api/members'</defaultValue>
      <description></description>
      <id>e3f70a07-6466-42ff-9733-3599d2e4bbdf</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
