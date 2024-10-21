<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OBJ.014.24 Post Create Sub Module Penyesuaian</name>
   <tag></tag>
   <elementGuidId>3eeaf90b-1f1b-4105-8714-9f32e19d1b0e</elementGuidId>
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
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\n  \&quot;moduleId\&quot;: \&quot;6703b210e79c6e916327a42b\&quot;,\n    \&quot;name\&quot;: \&quot;Setting V5\&quot;,\n    \&quot;isIncludeFrontend\&quot;: false,\n    \&quot;isIncludeMobile\&quot;: false,\n    \&quot;isIncludeIot\&quot;: false,\n    \&quot;team\&quot;: {\n        \&quot;uiux\&quot;: \&quot;66e907b5e64712c4e5eb101d\&quot;,\n        \&quot;devOps\&quot;: \&quot;66d1771c1ebe5344bc2b9dab\&quot;,\n        \&quot;qcWeb\&quot;: \&quot;66ea825884cd207e408ae6ce\&quot;,\n        \&quot;qcMobile\&quot;: \&quot;66ea825884cd207e408ae6ce\&quot;,\n        \&quot;backend\&quot;: [\n            \&quot;66eb9159b3bb95576adbe994\&quot;\n        ],\n        \&quot;frontend\&quot;: [\n            \&quot;66eb8e2eb3bb95576adbe6f7\&quot;\n        ]\n    },\n    \&quot;repository\&quot;: {\n        \&quot;backend\&quot;: [\n            \&quot;setting-backend\&quot;\n        ],\n        \&quot;frontend\&quot;: [\n            \&quot;setting-frontend\&quot;\n        ]\n    }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>5487e65f-d0cf-4a1b-a4cf-625dbdf03eee</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>60f6b833-27f6-44b4-a88e-3ecd2f89dd2d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>cd6557e0-49a9-41f2-9320-640a9ab25e24</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${submodul}${endpoint}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.submodul</defaultValue>
      <description></description>
      <id>ac8b7549-cccf-4668-adc0-c863a4ace6c2</id>
      <masked>false</masked>
      <name>submodul</name>
   </variables>
   <variables>
      <defaultValue>'66e901c165b533fbe0c72c03/project-detail/sub-module'</defaultValue>
      <description></description>
      <id>51091556-f0ed-4eab-b909-f3a62aea778d</id>
      <masked>false</masked>
      <name>endpoint</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>f9908293-3b0c-4e76-8e99-f8971a935b64</id>
      <masked>false</masked>
      <name>token</name>
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
